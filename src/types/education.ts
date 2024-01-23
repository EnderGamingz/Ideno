export interface EducationModel {
  id: number;
  userId: number;
  school: string;
  degree?: string;
  field?: string;
  startDate?: string;
  endDate?: string;
  createdAt: string;
}

export interface PublicEducationModel {
  school: string;
  degree?: string;
  field?: string;
  startDate?: string;
  endDate?: string;
}

export interface AddEducationPayload {
  school: string;
  degree?: string;
  field?: string;
  startDate?: string;
  endDate?: string;
}
