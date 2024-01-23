export interface UserModel {
  id: number;
  username: string;
  email: string;
  password: string;
  role: string;
  created_at: string;
}

export interface PublicAuthUserModel {
  id: number;
  username: string;
  email: string;
  created_at: string;
}

export interface Created {
  id?: number;
}

export interface AdminUpdateUserPayload {
  username: string;
  email: string;
  role: string;
}
