CREATE TABLE service_types_master (
    service_type_id INT AUTO_INCREMENT PRIMARY KEY,
    supports_alacarte BOOLEAN NOT NULL,
    supports_course BOOLEAN NOT NULL
);

-- 初期データ
INSERT INTO service_types_master (supports_alacarte, supports_course) VALUES
(TRUE, FALSE),  -- アラカルトのみ
(FALSE, TRUE),  -- コースのみ
(TRUE, TRUE);   -- アラカルトとコースの両方
