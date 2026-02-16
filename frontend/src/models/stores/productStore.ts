import { defineStore } from 'pinia';
import type { Product } from '../types';

interface ProductState {
  products: Product[];
  loading: boolean;
  error: string | null;
}

export const useProductStore = defineStore('product', {
  state: (): ProductState => ({
    products: [],
    loading: false,
    error: null,
  }),

  getters: {
    getProductById:
      (state) =>
      (id: number): Product | undefined => {
        return state.products.find((product) => product.id === id);
      },
    inStockProducts: (state): Product[] => {
      return state.products.filter((product) => product.stock > 0);
    },
    outOfStockProducts: (state): Product[] => {
      return state.products.filter((product) => product.stock === 0);
    },
  },

  actions: {
    async fetchProducts() {
      this.loading = true;
      this.error = null;

      try {
        const result = await window.webui.call('get_products');
        const response = JSON.parse(result);

        if (response.success) {
          this.products = response.data;
        } else {
          this.error = response.error;
        }
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Unknown error';
      } finally {
        this.loading = false;
      }
    },

    async createProduct(productData: Partial<Product>) {
      this.loading = true;
      this.error = null;

      try {
        const elementName = `create_product:${productData.name}:${productData.description}:${productData.price}:${productData.category}:${productData.stock}`;
        const result = await window.webui.call(elementName);
        const response = JSON.parse(result);

        if (response.success) {
          await this.fetchProducts();
          return response;
        } else {
          this.error = response.error;
          return response;
        }
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Unknown error';
        return { success: false, error: this.error };
      } finally {
        this.loading = false;
      }
    },

    async updateProduct(id: number, productData: Partial<Product>) {
      this.loading = true;
      this.error = null;

      try {
        const elementName = `update_product:${id}:${productData.name || ''}:${productData.description || ''}:${productData.price || ''}:${productData.category || ''}:${productData.stock || ''}`;
        const result = await window.webui.call(elementName);
        const response = JSON.parse(result);

        if (response.success) {
          await this.fetchProducts();
          return response;
        } else {
          this.error = response.error;
          return response;
        }
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Unknown error';
        return { success: false, error: this.error };
      } finally {
        this.loading = false;
      }
    },

    async deleteProduct(id: number) {
      this.loading = true;
      this.error = null;

      try {
        const elementName = `delete_product:${id}`;
        const result = await window.webui.call(elementName);
        const response = JSON.parse(result);

        if (response.success) {
          this.products = this.products.filter((product) => product.id !== id);
          return response;
        } else {
          this.error = response.error;
          return response;
        }
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Unknown error';
        return { success: false, error: this.error };
      } finally {
        this.loading = false;
      }
    },
  },
});
