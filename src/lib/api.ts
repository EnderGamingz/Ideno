import axios from 'axios';
import { cookies } from 'next/headers';
import { PublicAuthUserModel } from '@/types/user';

const api_url = `${process.env.API_URL}/api/${process.env.API_VERSION}/`;

axios.defaults.withCredentials = true;

function getServersideCookie() {
  const id = cookies().get('id');
  if (id) {
    return 'id=' + id.value;
  }

  return '';
}

export default class API {
  static async get(endpoint: string) {
    const response = await axios.get(api_url + endpoint, {
      headers: {
        Cookie: getServersideCookie(),
      },
    });
    return await response.data;
  }

  static async post(endpoint: string, data: any) {
    const response = await axios.post(api_url + endpoint, data, {
      headers: {
        Cookie: getServersideCookie(),
      },
    });
    return await response.data;
  }

  static async update(endpoint: string, data: any) {
    const response = await axios.patch(api_url + endpoint, data, {
      headers: {
        Cookie: getServersideCookie(),
      },
    });
    return await response.data;
  }

  static async delete(endpoint: string) {
    const response = await axios.delete(api_url + endpoint, {
      headers: {
        Cookie: getServersideCookie(),
      },
    });
    return await response.data;
  }

  static async login(username: string, password: string) {
    return await axios.post(api_url + 'auth/login', { username, password });
  }

  static async auth() {
    return await axios.get(api_url + 'auth', {
      headers: {
        Cookie: getServersideCookie(),
      },
    }).then(res => res.data as PublicAuthUserModel)
  }

  static async logout() {
    return await axios.get(api_url + 'auth/logout', {
      headers: {
        Cookie: getServersideCookie(),
      },
    });
  }
}
