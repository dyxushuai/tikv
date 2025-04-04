// Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

define_error_codes!(
    "KV:Coprocessor:",

    LOCKED => ("Locked", "", ""),
    DEADLINE_EXCEEDED => ("DeadlineExceeded", "", ""),
    MAX_PENDING_TASKS_EXCEEDED => ("MaxPendingTasksExceeded", "", ""),
    MEMORY_QUOTA_EXCEEDED => ("MemoryQuotaExceeded", "", ""),

    INVALID_DATA_TYPE => ("InvalidDataType", "", ""),
    ENCODING => ("Encoding", "", ""),
    COLUMN_OFFSET => ("ColumnOffset", "", ""),
    UNKNOWN_SIGNATURE => ("UnknownSignature", "", ""),
    EVAL => ("Eval", "", ""),
    CORRUPTED_DATA => ("CorruptedData", "", ""),

    STORAGE_ERROR => ("StorageError", "", ""),
    INVALID_CHARACTER_STRING => ("InvalidCharacterString", "", ""),

    INVALID_MAX_TS_UPDATE => ("InvalidMaxTsUpdate", "", ""),
    DEFAULT_NOT_FOUND => ("DefaultNotFound", "", "")
);
