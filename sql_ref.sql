create extension postgres_spice;
ALTER TABLE transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE transactions FORCE ROW LEVEL SECURITY;
CREATE POLICY viewable_by_id ON transactions FOR SELECT USING (
    (select has_permission('transaction',id :: varchar, 'user','someid', 'view'))
    );
alter role postgres nobypassrls;
alter role postgres nosuperuser;