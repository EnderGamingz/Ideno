
export interface ContactInformationModel {
  id: number;
  userId: number;
  typeField: string;
  value: string;
  createdAt: string;
}

export interface PublicContactInformationModel {
  typeField: string;
  value: string;
}