import { readonly } from 'vue';
import type { ApiResponse } from '../../types';
import { useApi } from '../api/apiClient.ts';

export function useDatabase() {
  const { callApi, loading, error } = useApi();

  const getUsers = async (): Promise<ApiResponse> => {
    return await callApi('get_users');
  };

  const createUser = async (userData: {
    name: string;
    email: string;
    role: string;
    status: string;
  }): Promise<ApiResponse> => {
    const elementName = `create_user:${userData.name}:${userData.email}:${userData.role}:${userData.status}`;
    return await callApi(elementName);
  };

  const updateUser = async (
    id: number,
    userData: { name?: string; email?: string; role?: string; status?: string }
  ): Promise<ApiResponse> => {
    const elementName = `update_user:${id}:${userData.name || ''}:${userData.email || ''}:${userData.role || ''}:${userData.status || ''}`;
    return await callApi(elementName);
  };

  const deleteUser = async (id: number): Promise<ApiResponse> => {
    const elementName = `delete_user:${id}`;
    return await callApi(elementName);
  };

  const getProducts = async (): Promise<ApiResponse> => {
    return await callApi('get_products');
  };

  const createProduct = async (productData: {
    name: string;
    description: string;
    price: number;
    category: string;
    stock: number;
  }): Promise<ApiResponse> => {
    const elementName = `create_product:${productData.name}:${productData.description}:${productData.price}:${productData.category}:${productData.stock}`;
    return await callApi(elementName);
  };

  const updateProduct = async (
    id: number,
    productData: {
      name?: string;
      description?: string;
      price?: number;
      category?: string;
      stock?: number;
    }
  ): Promise<ApiResponse> => {
    const elementName = `update_product:${id}:${productData.name || ''}:${productData.description || ''}:${productData.price || ''}:${productData.category || ''}:${productData.stock || ''}`;
    return await callApi(elementName);
  };

  const deleteProduct = async (id: number): Promise<ApiResponse> => {
    const elementName = `delete_product:${id}`;
    return await callApi(elementName);
  };

  return {
    loading: readonly(loading),
    error: readonly(error),
    getUsers,
    createUser,
    updateUser,
    deleteUser,
    getProducts,
    createProduct,
    updateProduct,
    deleteProduct,
  };
}
