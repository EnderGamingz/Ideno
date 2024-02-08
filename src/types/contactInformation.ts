export interface ContactInformationModel {
  id: number;
  user_id: number;
  type_field: string;
  value: string;
  created_at: string;
}

export interface PublicContactInformationModel {
  type_field: string;
  value: string;
}

export interface AuthContactInformationModel
  extends PublicContactInformationModel {
  id?: number;
}

export interface AddContactInformationPayload {
  contact_type: string;
  value: string;
}
