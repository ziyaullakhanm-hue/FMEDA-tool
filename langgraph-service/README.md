# LangGraph Service

Python-based microservice for AI-powered workflow orchestration using LangGraph and LangChain.

## Setup

### Local Development

1. Create a virtual environment:
```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
```

2. Install dependencies:
```bash
pip install -r requirements.txt
```

3. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your settings
```

4. Run the service:
```bash
python main.py
```

The service will be available at `http://localhost:8001`

### Docker

Run with docker-compose from the project root:
```bash
docker-compose up langgraph
```

## API Endpoints

### Health Check
```
GET /health
```

Returns service health status.

### Execute Workflow
```
POST /workflow/execute
Content-Type: application/json

{
  "workflow_type": "analysis",
  "data": {}
}
```

Returns workflow execution results.

## Configuration

Environment variables:
- `OPENAI_API_KEY`: OpenAI API key for LLM access
- `DATABASE_URL`: PostgreSQL connection string
- `LLM_MODEL`: LLM model to use (default: gpt-4)
- `DEBUG`: Enable debug logging (default: False)

## Project Structure

```
langgraph-service/
├── main.py              # FastAPI application
├── requirements.txt     # Python dependencies
├── .env.example         # Example environment variables
├── Dockerfile          # Docker image definition
├── workflows/          # LangGraph workflow definitions (TODO)
├── agents/             # AI agent implementations (TODO)
└── README.md           # This file
```

## Next Steps

1. Define specific workflows in `workflows/` directory
2. Implement AI agents in `agents/` directory
3. Connect to your FMEDA analysis logic
4. Add authentication and authorization
5. Implement workflow persistence and state management
