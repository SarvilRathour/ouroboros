pub struct TaskCreateRequest{
    title:String,
    description :String,
    status:TaskStatus,
    priority:TaskPriority,
    created_by_id UUID NOT NULL REFERENCES users(id),
    assigned_to_id UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
}