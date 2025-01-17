export interface CertificationModel {
  id: number;
  user_id: number;
  name: string;
  organization: string;
  issue_date?: string;
  expiration_date?: string;
  credential_id?: string;
  credential_url?: string;
  created_at: string;
}

export interface PublicCertificationModel {
  name: string;
  organization: string;
  issue_date?: string;
  expiration_date?: string;
  credential_id?: string;
  credential_url?: string;
}

export interface AuthCertificationModel extends PublicCertificationModel {
  id?: number;
}

export interface AddCertificationPayload {
  name: string;
  organization: string;
  issueDate?: string;
  expiration_date?: string;
  credential_id?: string;
  credential_url?: string;
}
