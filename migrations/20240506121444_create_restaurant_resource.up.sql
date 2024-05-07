CREATE TABLE restaurant_resource (
    id INT AUTO_INCREMENT PRIMARY KEY,
    restaurant_id VARCHAR(10) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    name_id INT,
    payment_method_id INT,
    service_type_id INT,
    FOREIGN KEY (name_id) REFERENCES restaurant_name_master(name_id),
    FOREIGN KEY (payment_method_id) REFERENCES payment_methods_master(payment_method_id),
    FOREIGN KEY (service_type_id) REFERENCES service_types_master(service_type_id),
    INDEX idx_restaurant_id (restaurant_id)
);

-- 初期データ
INSERT INTO restaurant_resource (restaurant_id, created_at, name_id, payment_method_id, service_type_id) VALUES
('0000000001', NOW(), 1, 1, 1), -- レストランA: 現金のみ、アラカルトのみ
('0000000002', NOW(), 2, 2, 2), -- レストランB: カードのみ、コースのみ
('0000000003', NOW(), 3, 3, 3), -- レストランC: 現金とカード、アラカルトとコース
('0000000004', NOW(), 4, 1, 3), -- レストランD: 現金のみ、アラカルトとコース
('0000000005', NOW(), 5, 2, 3); -- レストランE: カードのみ、アラカルトとコース
