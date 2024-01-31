export interface ExperienceModel {
  id: number;
  user_id: number;
  company: string;
  title: string;
  start_date?: string;
  end_date?: string;
  exp_type?: string;
  description?: string;
  created_at: string;
}

export interface PublicExperienceModel {
  company: string;
  title: string;
  start_date?: string;
  end_date?: string;
  exp_type?: string;
  description?: string;
}

export interface AuthExperienceModel extends PublicExperienceModel {
  id?: number;
}

export interface AddExperiencePayload {
  company: string;
  title: string;
  start_date?: string;
  end_date?: string;
  exp_type?: string;
  description?: string;
}
