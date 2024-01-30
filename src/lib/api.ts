import axios from 'axios';
import { cookies } from 'next/headers';
import {
  AdminUpdateUserPayload,
  Created,
  PublicAuthUserModel,
  UserModel,
} from '@/types/user';
import {
  ProfileModel,
  ProfileUpdatePayload,
  PublicProfileResponse,
} from '@/types/profile';
import { AccountUpdatePayload, PasswordUpdatePayload } from '@/types/account';
import { redirect } from 'next/navigation';
import {
  AddContactInformationPayload,
  ContactInformationModel,
} from '@/types/contactInformation';
import {
  AddCertificationPayload,
  AuthCertificationModel,
} from '@/types/certification';
import { AddEducationPayload } from '@/types/education';
import { AddExperiencePayload } from '@/types/experience';

const api_url = `${process.env.API_URL}/api/${process.env.API_VERSION}/`;

axios.defaults.withCredentials = true;

function getServersideCookie() {
  const id = cookies().get('id');
  return id ? `id=${id.value}` : '';
}

let profile_auth_api = {
  async get() {
    return await API.get('auth/profile').then(
      async res => (await res.json()) as ProfileModel,
    );
  },

  async update(data: ProfileUpdatePayload) {
    return await API.patch('auth/profile', data).then(
      async res => (await res.json()) as ProfileModel,
    );
  },

  contactInformation: {
    async get() {
      return await API.get('auth/profile/contact-information').then(
        async res => (await res.json()) as ContactInformationModel[],
      );
    },
    async create(data: AddContactInformationPayload) {
      return await API.post('auth/profile/contact-information', data).then(
        async res => (await res.json()) as Created,
      );
    },
    async deleteById(id: number) {
      return await API.delete(`auth/profile/contact-information/${id}`);
    },
    async updateById(id: number, data: AddContactInformationPayload) {
      return await API.patch(
        `auth/profile/contact-information/${id}`,
        data,
      ).then(async res => (await res.json()) as ContactInformationModel);
    },
  },
  certification: {
    async get() {
      return await API.get('auth/profile/certification').then(
        async res => (await res.json()) as ContactInformationModel[],
      );
    },
    async create(data: AddCertificationPayload) {
      return await API.post('auth/profile/certification', data).then(
        async res => (await res.json()) as Created,
      );
    },
    async deleteById(id: number) {
      return await API.delete(`auth/profile/certification/${id}`);
    },
    async updateById(id: number, data: AddCertificationPayload) {
      return await API.patch(`auth/profile/certification/${id}`, data).then(
        async res => (await res.json()) as ContactInformationModel,
      );
    },
  },
  education: {
    async get() {
      return await API.get('auth/profile/education').then(
        async res => (await res.json()) as ContactInformationModel[],
      );
    },
    async create(data: AddEducationPayload) {
      return await API.post('auth/profile/education', data).then(
        async res => (await res.json()) as Created,
      );
    },
    async deleteById(id: number) {
      return await API.delete(`auth/profile/education/${id}`);
    },
    async updateById(id: number, data: AddEducationPayload) {
      return await API.patch(`auth/profile/education/${id}`, data).then(
        async res => (await res.json()) as ContactInformationModel,
      );
    },
  },
  experience: {
    async get() {
      return await API.get('auth/profile/experience').then(
        async res => (await res.json()) as ContactInformationModel[],
      );
    },
    async create(data: AddExperiencePayload) {
      return await API.post('auth/profile/experience', data).then(
        async res => (await res.json()) as Created,
      );
    },
    async deleteById(id: number) {
      return await API.delete(`auth/profile/experience/${id}`);
    },
    async updateById(id: number, data: AddExperiencePayload) {
      return await API.patch(`auth/profile/experience/${id}`, data).then(
        async res => (await res.json()) as ContactInformationModel,
      );
    },
  },
};

const admin_api = {
  user: {
    async getAll() {
      return await API.get('auth/admin/users').then(
        async res => (await res.json()) as UserModel[],
      );
    },
    async getById(id: number) {
      return await API.get(`auth/admin/users/${id}`).then(
        async res => (await res.json()) as UserModel,
      );
    },
    async deleteById(id: number) {
      return await API.delete(`auth/admin/users/${id}`).then(
        async res => (await res.json()) as UserModel,
      );
    },
    async updateById(id: number, data: AdminUpdateUserPayload) {
      return await API.patch(`auth/admin/users/${id}`, data).then(
        async res => (await res.json()) as UserModel,
      );
    },
  },
  certification: {
    async deleteById(id: number) {
      return await API.delete(`auth/admin/certification/${id}`).then(
        async res => (await res.json()) as UserModel,
      );
    },
  },
  education: {
    async deleteById(id: number) {
      return await API.delete(`auth/admin/education/${id}`).then(
        async res => (await res.json()) as UserModel,
      );
    },
  },
  experience: {
    async deleteById(id: number) {
      return await API.delete(`auth/admin/experience/${id}`).then(
        async res => (await res.json()) as UserModel,
      );
    },
  },
  contactInformation: {
    async deleteById(id: number) {
      return await API.delete(`auth/admin/contact-information/${id}`).then(
        async res => (await res.json()) as UserModel,
      );
    },
  },
};

const user_api = {
  async auth() {
    return await API.get('auth', { tags: ['auth'], revalidate: 10 }).then(
      async res => {
        if (!res.ok) return undefined;

        const data = await res.json();
        return data as PublicAuthUserModel;
      },
    );
  },

  async login(username: string, password: string) {
    return await API.post('auth/login', { username, password });
  },

  async register(username: string, email: string, password: string) {
    return await API.post('auth/register', { username, email, password });
  },

  async logout() {
    return await API.get('auth/logout');
  },

  account: {
    async update(data: AccountUpdatePayload) {
      return await API.patch('auth/account', data).then(
        async res => (await res.json()) as ProfileModel,
      );
    },
    async delete() {
      API.delete('/auth/account').then(res => {
        if (res.ok) redirect('/api/session/clear');
      });
    },
  },

  password: {
    async update(data: PasswordUpdatePayload) {
      return await API.patch('auth/password', data).then(
        async res => (await res.json()) as ProfileModel,
      );
    },
  },

  profile: profile_auth_api,

  admin: admin_api,
};

const profile_public_api = {
  async getByUsername(name: string) {
    return await API.get(`profile/${name}`).then(async res => {
      if (!res.ok) return undefined;
      return (await res.json()) as PublicProfileResponse;
    });
  },
  async getCertificationsByUsername(name: string) {
    return await API.get(`profile/${name}/certifications`).then(async res => {
      if (!res.ok) return undefined;
      return (await res.json()) as AuthCertificationModel[];
    });
  },
};

export default class API {
  static async get(endpoint: string, options?: any) {
    return await fetch(api_url + endpoint, {
      method: 'GET',
      headers: {
        Cookie: getServersideCookie(),
      },
      next: options,
    });
  }

  static async post(endpoint: string, data: any) {
    return await fetch(api_url + endpoint, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Cookie: getServersideCookie(),
      },
      body: JSON.stringify(data),
    });
  }

  static async patch(endpoint: string, data: any) {
    return await fetch(api_url + endpoint, {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
        Cookie: getServersideCookie(),
      },
      body: JSON.stringify(data),
    });
  }

  static async delete(endpoint: string) {
    return await fetch(api_url + endpoint, {
      method: 'DELETE',
      headers: {
        Cookie: getServersideCookie(),
      },
    });
  }

  static auth = user_api;
  static profile = profile_public_api;
}
