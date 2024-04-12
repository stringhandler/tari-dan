-- Latests scanned blocks, separatedly by committee (epoch + shard)
-- Used mostly for effient scanning of events in the whole network
create table scanned_block_ids
(
    id               integer    not NULL primary key AUTOINCREMENT,
    epoch            bigint     not NULL,
    shard            bigint     not null,
    last_block_id    blob       not null
);


-- There should only be one last scanned block by committee (epoch + shard)
create unique index scanned_block_ids_unique_commitee on scanned_block_ids (epoch, shard);

-- DB index for faster retrieval of the latest block by committee
create index scanned_block_ids_commitee on scanned_block_ids (epoch, shard);