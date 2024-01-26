import { PublicCertificationModel } from '@/types/certification';
import { PublicEducationModel } from '@/types/education';
import { PublicExperienceModel } from '@/types/experience';
import { PublicContactInformationModel } from '@/types/contactInformation';

export interface ProfileModel {
  userId: number;
  first_name?: string;
  last_name?: string;
  pronouns?: string;
  headline?: string;
  country?: string;
  city?: string;
  bio?: string;
  createdAt: string;
}

export interface PublicProfileModel {
  first_name?: string;
  last_name?: string;
  pronouns?: string;
  headline?: string;
  country?: string;
  city?: string;
  bio?: string;
}

export interface PublicProfileResponse {
  profile: PublicProfileModel;
  certification: PublicCertificationModel[];
  education: PublicEducationModel[];
  experience: PublicExperienceModel[];
  contactInformation: PublicContactInformationModel[];
}

export interface ProfileUpdatePayload {
  first_name?: string;
  last_name?: string;
  pronouns?: string;
  headline?: string;
  country?: string;
  city?: string;
  bio?: string;
}
