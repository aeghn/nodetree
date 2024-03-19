import axios, { AxiosInstance, AxiosRequestConfig } from "axios";

axios.defaults.baseURL =
  window.location.protocol + "//" + window.location.host + "/api";
console.log("base url: " + axios.defaults.baseURL);

export const createInstance = (config: AxiosRequestConfig): AxiosInstance => {
  return axios.create(config);
};

export const get = async (url: string, config?: AxiosRequestConfig) => {
  const _instance = createInstance(config || {});

  const res = await _instance.get(url, config);
  return res;
};

export const post = async (
  url: string,
  data: any,
  config?: AxiosRequestConfig
) => {
  const _instance = createInstance(config || {});

  const res = await _instance.post(url, data, config);
  return res;
};

export const put = async (
  url: string,
  data: any,
  config?: AxiosRequestConfig
) => {
  const _instance = createInstance(config || {});

  const res = await _instance.put(url, data, config);
  return res;
};

export const _delete = async (url: string, config?: AxiosRequestConfig) => {
  const _instance = createInstance(config || {});

  const res = await _instance.delete(url, config);
  return res;
};

export const request = {
  get,
  post,
  put,
  delete: _delete,
};
