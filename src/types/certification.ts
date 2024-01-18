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