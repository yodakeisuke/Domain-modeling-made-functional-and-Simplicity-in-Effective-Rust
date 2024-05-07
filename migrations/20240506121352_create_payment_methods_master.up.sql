CREATE TABLE payment_methods_master (
    payment_method_id INT AUTO_INCREMENT PRIMARY KEY,
    supports_cash BOOLEAN NOT NULL,
    supports_card BOOLEAN NOT NULL
);

-- 初期データ
INSERT INTO payment_methods_master (supports_cash, supports_card) VALUES
(TRUE, FALSE),  -- 現金のみ
(FALSE, TRUE),  -- カードのみ
(TRUE, TRUE);   -- 現金とカードの両方
