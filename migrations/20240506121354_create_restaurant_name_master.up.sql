CREATE TABLE restaurant_name_master (
    name_id INT AUTO_INCREMENT PRIMARY KEY,
    restaurant_name VARCHAR(255) NOT NULL
);

-- 初期データ
INSERT INTO restaurant_name_master (restaurant_name) VALUES
('サイゼリヤ'), ('吉野家'), ('マック'), ('ケンタ'), ('レストランE');
