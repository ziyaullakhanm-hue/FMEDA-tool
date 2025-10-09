INSERT INTO failure_modes (mpn, family, mode, lambda, detection_coverage)
VALUES
('RES-0603-1K','resistor','open', 50.0, 0.0),
(NULL,'capacitor','short', 10.0, 0.0),
(NULL,'ic','total', 300.0, 0.0)
ON CONFLICT DO NOTHING;
