export interface ExperienceModel {
  id: number;
  userId: number;
  company: string;
  title: string;
  startDate?: string;
  endDate?: string;
  expType?: string;
  description?: string;
  createdAt: string;
}

export interface PublicExperienceModel {
  company: string;
  title: string;
  startDate?: string;
  endDate?: string;
  expType?: string;
  description?: string;
}