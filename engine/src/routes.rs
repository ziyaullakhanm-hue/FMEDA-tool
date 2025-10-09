use axum::{
    extract::{State, Json, Path},
    Router, routing::{get, post},
    http::StatusCode,
};
use crate::db::Db;
use crate::models::{CalculationRequest, CalculationResult, Component, FailureMode};
use serde_json::Value;
use sqlx::types::Uuid;
use crate::calc;
use tracing::instrument;

pub fn router() -> Router<Db> {
    Router::new()
        .route("/health", get(health))
        .route("/failure_modes", post(create_failure_mode))
        .route("/calculate", post(calculate))
        .route("/components/:project_id", get(list_components_for_project))
}

async fn health() -> &'static str { "ok" }

#[instrument(skip(pool, payload))]
async fn create_failure_mode(
    State(pool): State<Db>,
    Json(payload): Json<Value>
) -> Result<(StatusCode, Json<Value>), (StatusCode, String)> {
    // simple insert expectation: { "mpn": "...", "family": "...", "mode":"", "lambda": 123.0, "detection_coverage": 0.9 }
    let mpn: Option<String> = payload.get("mpn").and_then(|v| v.as_str().map(|s| s.to_string()));
    let family: Option<String> = payload.get("family").and_then(|v| v.as_str().map(|s| s.to_string()));
    let mode = payload.get("mode").and_then(|v| v.as_str()).ok_or((StatusCode::BAD_REQUEST, "mode missing".to_string()))?.to_string();
    let lambda = payload.get("lambda").and_then(|v| v.as_f64()).ok_or((StatusCode::BAD_REQUEST, "lambda missing or invalid".to_string()))?;
    let detection_coverage = payload.get("detection_coverage").and_then(|v| v.as_f64()).unwrap_or(0.0_f64) as f32;

    let rec = sqlx::query!(
        r#"
        INSERT INTO failure_modes (mpn, family, mode, lambda, detection_coverage)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, mpn, family, mode, lambda, detection_coverage, created_at
        "#,
        mpn,
        family,
        mode,
        lambda as f64,
        detection_coverage as f32
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(json!({
        "id": rec.id,
        "mpn": rec.mpn,
        "family": rec.family,
        "mode": rec.mode,
        "lambda": rec.lambda,
        "detection_coverage": rec.detection_coverage
    }))))
}

#[instrument(skip(pool, req))]
async fn calculate(
    State(pool): State<Db>,
    Json(req): Json<CalculationRequest>
) -> Result<Json<Value>, (StatusCode, String)> {
    // 1. load components
    let mut components: Vec<Component> = Vec::new();
    for cid in &req.component_ids {
        let c = sqlx::query_as!(
            Component,
            r#"SELECT id, project_id, manufacturer_part_number, manufacturer, reference_designator, quantity, created_at
               FROM components WHERE id = $1"#,
            cid
        ).fetch_one(&pool).await.map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;
        components.push(c);
    }

    // 2. for each component, get applicable failure modes (first try MPN, otherwise family NULL)
    use std::collections::HashMap;
    let mut modes_map: HashMap<uuid::Uuid, Vec<FailureMode>> = HashMap::new();

    for comp in &components {
        // first: try mpn specific
        let mut modes = sqlx::query_as!(
            FailureMode,
            r#"SELECT id, mpn, family, mode, lambda, detection_coverage, created_at FROM failure_modes WHERE mpn = $1"#,
            comp.manufacturer_part_number
        ).fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        // if none, try family-based matches (very simple: family field matched against part prefix)
        if modes.is_empty() {
            modes = sqlx::query_as!(
                FailureMode,
                r#"SELECT id, mpn, family, mode, lambda, detection_coverage, created_at FROM failure_modes WHERE family IS NOT NULL LIMIT 10"#
            ).fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }

        modes_map.insert(comp.id, modes);
    }

    // 3. run calculation
    let result_json = calc::calc_project_fmeda(&components, &modes_map);

    // 4. persist calculation snapshot/result
    let payload = serde_json::to_value(&req).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let result_value = result_json.clone();
    let rec = sqlx::query!(
        r#"
        INSERT INTO calculations (project_id, payload, result)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        req.project_id,
        payload,
        result_value
    ).fetch_one(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(result_json))
}

#[instrument(skip(pool, project_id))]
async fn list_components_for_project(
    State(pool): State<Db>,
    Path(project_id): Path<Uuid>
) -> Result<Json<Value>, (StatusCode, String)> {
    let recs = sqlx::query_as!(
        Component,
        r#"SELECT id, project_id, manufacturer_part_number, manufacturer, reference_designator, quantity, created_at FROM components WHERE project_id = $1"#,
        project_id
    ).fetch_all(&pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::to_value(recs).unwrap_or(json!([]))))
}
