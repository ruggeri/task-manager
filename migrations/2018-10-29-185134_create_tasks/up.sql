CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  title VARCHAR(1024) NOT NULL,
  status TASK_STATUS NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

INSERT INTO tasks (title, status)
VALUES
  ('Fix lambda comments', 'available_to_perform'),
  ('Launch lecture on Godel', 'available_to_perform'),
  ('Find Zumba times', 'available_to_perform'),
  ('Write Anastassia part D', 'available_to_perform'),
  ('Finish taxes payment', 'available_to_perform'),
  ('Look into glasses', 'available_to_perform'),
  ('Schedule flu shot', 'available_to_perform'),
  ('Review Google Photos subscription', 'available_to_perform'),
  ('Write round-robin productivity software', 'available_to_perform'),
  ('Review budget', 'available_to_perform'),
  ('Get drysack from Clark', 'available_to_perform'),
  ('Get coffee with employees', 'available_to_perform'),
  ('Think about career/life direction', 'available_to_perform'),
  ('Buy distilled water', 'available_to_perform'),
  ('Look into rental with JRN', 'available_to_perform'),
  ('Figure out rent question with Andrew', 'available_to_perform'),
  ('Get Anastassia plant 2x', 'available_to_perform'),
  ('Review meditation retreats', 'available_to_perform'),
  ('Purchase singing bowls', 'available_to_perform'),
  ('Finish Zion planning', 'available_to_perform'),
  ('Charge Kate money', 'available_to_perform'),
  ('Look into Kate''s slack history question', 'available_to_perform'),
  ('Write Diane Hermann/Craig Abbey', 'available_to_perform'),
  ('Buy a new baloon for Anastassia', 'available_to_perform')
;
