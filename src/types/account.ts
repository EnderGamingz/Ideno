export interface AccountUpdatePayload {
  username?: string;
  email?: string;
}

export interface PasswordUpdatePayload {
  old_password: string;
  new_password: string;
}
