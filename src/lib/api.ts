import axios from 'axios';
import { cookies } from 'next/headers';
import { PublicAuthUserModel } from '@/types/user';
import { ProfileModel, ProfileUpdatePayload } from '@/types/profile';

const api_url = `${process.env.API_URL}/api/${process.env.API_VERSION}/`;

axios.defaults.withCredentials = true;

function getServersideCookie() {
  const id = cookies().get('id');
  if (id) {
    return 'id=' + id.value;
  }

  return '';
}

const user_api = {
  async login(username: string, password: string) {
    return await API.post('auth/login', { username, password });
  },

  async register(username: string, email: string, password: string) {
    return await API.post('auth/register', { username, email, password });
  },

  async auth() {
    return await API.get('auth').then(res => res.data as PublicAuthUserModel);
  },

  async logout() {
    return await API.get('auth/logout');
  },

  profile: {
    async update(data: ProfileUpdatePayload) {
      return await API.patch('auth/profile', data).then(
        res => res.data as ProfileModel,
      );
    },
  },
};

export default class API {
  static async get(endpoint: string) {
    return await axios.get(api_url + endpoint, {
      headers: {
        Cookie: getServersideCookie(),
      },
    });
  }

  static async post(endpoint: string, data: any) {
    return await axios.post(api_url + endpoint, data, {
      headers: {
        Cookie: getServersideCookie(),
      },
    });
  }

  static async patch(endpoint: string, data: any) {
    return await axios.patch(api_url + endpoint, data, {
      headers: {
        Cookie: getServersideCookie(),
      },
    });
  }

  static async delete(endpoint: string) {
    return await axios.delete(api_url + endpoint, {
      headers: {
        Cookie: getServersideCookie(),
      },
    });
  }

  static auth = user_api;
}
