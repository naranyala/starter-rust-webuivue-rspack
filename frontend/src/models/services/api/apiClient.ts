import { readonly, ref } from 'vue';
import type { ApiResponse } from '../types';

export function useApi() {
  const loading = ref(false);
  const error = ref<string | null>(null);

  const callApi = async (methodName: string, data?: unknown): Promise<ApiResponse> => {
    loading.value = true;
    error.value = null;

    try {
      const result = await window.webui.call(methodName, data ? JSON.stringify(data) : '');
      const parsedResult = JSON.parse(result) as ApiResponse;

      return parsedResult;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Unknown error';
      error.value = errorMessage;
      return { success: false, error: errorMessage };
    } finally {
      loading.value = false;
    }
  };

  return {
    loading: readonly(loading),
    error: readonly(error),
    callApi,
  };
}
