CREATE TABLE tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    ip_address TEXT NOT NULL,
    user_agent TEXT NOT NULL,
    replaced_by UUID REFERENCES tokens(id) ON DELETE SET NULL,
    previous_token_id UUID REFERENCES tokens(id) ON DELETE SET NULL
);
