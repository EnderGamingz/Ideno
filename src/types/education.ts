export interface EducationModel {
  id: number;
  userId: number;
  school: string;
  degree?: string;
  field?: string;
  start_date?: string;
  end_date?: string;
  created_at: string;
}

export interface PublicEducationModel {
  school: string;
  degree?: string;
  field?: string;
  start_date?: string;
  end_date?: string;
}

export interface AuthEducationModel extends PublicEducationModel {
  id?: number;
}

export interface AddEducationPayload {
  school: string;
  degree?: string;
  field?: string;
  start_date?: string;
  end_date?: string;
}
