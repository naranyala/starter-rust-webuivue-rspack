import { computed, onMounted, ref } from 'vue';
import { useProductStore } from '../models/stores';
import type { Product } from '../models/types';

export function useProductListViewModel() {
  const productStore = useProductStore();
  const showAddProductModal = ref(false);
  const searchTerm = ref('');

  const currentProduct = ref<Partial<Product>>({
    id: null,
    name: '',
    description: '',
    price: 0,
    category: '',
    stock: 0,
    sku: '',
    weight: 0,
  });

  const filteredProducts = computed(() => {
    if (!searchTerm.value) return productStore.products;
    const term = searchTerm.value.toLowerCase();
    return productStore.products.filter(
      (product) =>
        product.name.toLowerCase().includes(term) ||
        product.description.toLowerCase().includes(term) ||
        product.category.toLowerCase().includes(term) ||
        product.sku?.toLowerCase().includes(term)
    );
  });

  const inStockCount = computed(() => productStore.products.filter((p) => p.stock > 0).length);
  const outOfStockCount = computed(() => productStore.products.filter((p) => p.stock <= 0).length);
  const totalValue = computed(() =>
    productStore.products.reduce((sum, p) => sum + p.price * p.stock, 0).toFixed(2)
  );

  const loadProducts = async () => {
    await productStore.fetchProducts();
  };

  const editProduct = (product: Product) => {
    currentProduct.value = { ...product };
    showAddProductModal.value = true;
  };

  const deleteProduct = async (id: number) => {
    if (confirm('Are you sure you want to delete this product?')) {
      await productStore.deleteProduct(id);
    }
  };

  const saveProduct = async () => {
    try {
      if (currentProduct.value.id) {
        await productStore.updateProduct(currentProduct.value.id, currentProduct.value);
      } else {
        await productStore.createProduct(currentProduct.value);
      }
      closeModal();
    } catch (err) {
      console.error('Error saving product:', err);
    }
  };

  const closeModal = () => {
    showAddProductModal.value = false;
    currentProduct.value = {
      id: null,
      name: '',
      description: '',
      price: 0,
      category: '',
      stock: 0,
      sku: '',
      weight: 0,
    };
  };

  const truncateDescription = (desc?: string, maxLength = 50): string => {
    if (!desc) return '';
    return desc.length > maxLength ? desc.substring(0, maxLength) + '...' : desc;
  };

  const getProductInitials = (name: string): string => {
    return name
      .split(' ')
      .map((n) => n.charAt(0))
      .join('')
      .toUpperCase()
      .substring(0, 2);
  };

  const getStatusClass = (stock: number): string => {
    if (stock <= 0) return 'out-of-stock';
    if (stock <= 5) return 'low-stock';
    return 'in-stock';
  };

  const getStatusText = (stock: number): string => {
    if (stock <= 0) return 'Out of Stock';
    if (stock <= 5) return 'Low Stock';
    return 'In Stock';
  };

  onMounted(() => {
    loadProducts();
  });

  return {
    productStore,
    showAddProductModal,
    searchTerm,
    currentProduct,
    filteredProducts,
    inStockCount,
    outOfStockCount,
    totalValue,
    editProduct,
    deleteProduct,
    saveProduct,
    closeModal,
    truncateDescription,
    getProductInitials,
    getStatusClass,
    getStatusText,
    loadProducts,
  };
}
