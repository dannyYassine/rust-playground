INSERT INTO users (name, email, password) VALUES
    ('Alice Martin',   'alice@example.com',   'hashed_password_1'),
    ('Bob Stevens',    'bob@example.com',     'hashed_password_2'),
    ('Carol White',    'carol@example.com',   'hashed_password_3'),
    ('David Kim',      'david@example.com',   'hashed_password_4'),
    ('Eva Rossi',      'eva@example.com',     'hashed_password_5'),
    ('Frank Müller',   'frank@example.com',   'hashed_password_6'),
    ('Grace Chen',     'grace@example.com',   'hashed_password_7'),
    ('Henry Patel',    'henry@example.com',   'hashed_password_8'),
    ('Isla Nguyen',    'isla@example.com',     'hashed_password_9'),
    ('James O''Brien', 'james@example.com',   'hashed_password_10')
ON CONFLICT (email) DO NOTHING;
