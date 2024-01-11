create table if not exists liquidations (
    id text not null constraint liquidations_pk primary key,
    trx_hash text not null,
    liquidator text not null,
    liquidated_address text not null,
    debt_to_cover text not null,
    liquidated_collateral_amount text not null,
    receive_a_token text not null,
    block_num int not null,
    timestamp text not null
);
