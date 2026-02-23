<template>
  <div class="product-list">
    <div class="product-list-header">
      <h2>📦 Product Catalog</h2>
      <div class="header-actions">
        <input v-model="searchTerm" type="text" placeholder="Search products..." class="search-input" />
        <Button variant="primary" @click="showAddProductModal = true">+ Add Product</Button>
      </div>
    </div>

    <div v-if="productStore.loading" class="loading">Loading products...</div>
    <div v-else-if="productStore.error" class="error">{{ productStore.error }}</div>
    <div v-else class="product-content">
      <div class="content-wrapper">
        <div class="article-content">
          <h3>About Product Management</h3>
          <p>Effective product management is essential for businesses to maintain competitive advantage and meet customer needs.</p>
          <p>Modern product management encompasses market research, product development, pricing strategies, and inventory management.</p>
        </div>
        <div class="stats-summary">
          <div class="stat-card">
            <div class="stat-number">{{ filteredProducts.length }}</div>
            <div class="stat-label">Total Products</div>
          </div>
          <div class="stat-card">
            <div class="stat-number">{{ inStockCount }}</div>
            <div class="stat-label">In Stock</div>
          </div>
          <div class="stat-card">
            <div class="stat-number">{{ outOfStockCount }}</div>
            <div class="stat-label">Out of Stock</div>
          </div>
          <div class="stat-card">
            <div class="stat-number">${{ totalValue }}</div>
            <div class="stat-label">Total Value</div>
          </div>
        </div>
        <div class="product-table-container">
          <table class="product-table">
            <thead>
              <tr>
                <th>ID</th><th>Product</th><th>Description</th><th>Price</th><th>Category</th><th>Stock</th><th>Status</th><th>Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="product in filteredProducts" :key="product.id">
                <td>{{ product.id }}</td>
                <td>
                  <div class="product-info">
                    <div class="product-image-placeholder">{{ getProductInitials(product.name) }}</div>
                    <div>
                      <div class="product-name">{{ product.name }}</div>
                      <div class="product-sku">SKU: {{ product.sku || 'N/A' }}</div>
                    </div>
                  </div>
                </td>
                <td>{{ truncateDescription(product.description) }}</td>
                <td>${{ product.price.toFixed(2) }}</td>
                <td>{{ product.category }}</td>
                <td>{{ product.stock }}</td>
                <td><span :class="['status-badge', getStatusClass(product.stock)]">{{ getStatusText(product.stock) }}</span></td>
                <td>
                  <Button variant="secondary" @click="editProduct(product)" size="small">Edit</Button>
                  <Button variant="danger" @click="deleteProduct(product.id)" size="small">Delete</Button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <div v-if="showAddProductModal" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3 class="modal-title">{{ currentProduct.id ? 'Edit Product' : 'Add New Product' }}</h3>
          <button class="modal-close" @click="closeModal">&times;</button>
        </div>
        <div class="modal-body">
          <form @submit.prevent="saveProduct">
            <div class="form-row">
              <div class="form-group"><label>Product Name</label><input v-model="currentProduct.name" type="text" placeholder="Enter product name" required /></div>
              <div class="form-group"><label>SKU</label><input v-model="currentProduct.sku" type="text" placeholder="Enter SKU" /></div>
            </div>
            <div class="form-group"><label>Description</label><textarea v-model="currentProduct.description" placeholder="Enter product description" rows="3"></textarea></div>
            <div class="form-row">
              <div class="form-group"><label>Price</label><input v-model.number="currentProduct.price" type="number" step="0.01" placeholder="Enter price" required /></div>
              <div class="form-group"><label>Category</label><select v-model="currentProduct.category" required><option value="">Select</option><option>Electronics</option><option>Clothing</option><option>Books</option></select></div>
            </div>
            <div class="form-row">
              <div class="form-group"><label>Stock</label><input v-model.number="currentProduct.stock" type="number" placeholder="Stock" required /></div>
              <div class="form-group"><label>Weight</label><input v-model.number="currentProduct.weight" type="number" step="0.01" placeholder="Weight" /></div>
            </div>
            <div class="modal-footer">
              <Button variant="secondary" @click="closeModal">Cancel</Button>
              <Button variant="primary" :loading="productStore.loading">{{ currentProduct.id ? 'Update' : 'Create' }}</Button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import Button from '../../components/common/Button.vue';
import { useProductListViewModel } from '../../viewmodels/useProductListViewModel.ts';

const {
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
} = useProductListViewModel();
</script>

<style scoped>
.product-list { padding: 1rem; height: 100%; display: flex; flex-direction: column; }
.product-list-header { display: flex; flex-direction: column; gap: 1rem; margin-bottom: 1rem; }
.header-actions { display: flex; gap: 0.5rem; }
.search-input { flex: 1; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 0.375rem; }
.stats-summary { display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 1rem; margin-bottom: 1.5rem; }
.stat-card { background: white; border-radius: 0.5rem; padding: 1rem; text-align: center; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
.stat-number { font-size: 1.5rem; font-weight: 700; color: #4f46e5; }
.stat-label { font-size: 0.875rem; color: #6b7280; }
.product-content { flex: 1; display: flex; flex-direction: column; }
.product-table-container { flex: 1; overflow-x: auto; }
.product-table { width: 100%; border-collapse: collapse; background: white; border-radius: 0.5rem; min-width: 800px; }
.product-table th, .product-table td { padding: 0.75rem; text-align: left; border-bottom: 1px solid #e5e7eb; }
.product-table th { background: #f3f4f6; font-weight: 600; }
.product-info { display: flex; align-items: center; gap: 0.75rem; }
.product-image-placeholder { width: 40px; height: 40px; border-radius: 0.25rem; background: #4f46e5; color: white; display: flex; align-items: center; justify-content: center; }
.product-name { font-weight: 600; }
.product-sku { font-size: 0.75rem; color: #6b7280; }
.status-badge { padding: 0.25rem 0.5rem; border-radius: 9999px; font-size: 0.75rem; }
.status-badge.in-stock { background: #dcfce7; color: #16a34a; }
.status-badge.low-stock { background: #fef3c7; color: #d97706; }
.status-badge.out-of-stock { background: #fee2e2; color: #dc2626; }
.loading, .error { padding: 1rem; text-align: center; }
.error { color: #dc2626; background: #fee2e2; border-radius: 0.5rem; }
.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000; }
.modal-content { background: white; border-radius: 0.5rem; width: 100%; max-width: 600px; max-height: 90vh; overflow-y: auto; }
.modal-header { display: flex; justify-content: space-between; padding: 1rem; border-bottom: 1px solid #e5e7eb; }
.modal-title { font-size: 1.25rem; font-weight: 600; margin: 0; }
.modal-close { background: none; border: none; font-size: 1.5rem; cursor: pointer; }
.modal-body { padding: 1rem; }
.form-row { display: flex; gap: 1rem; margin-bottom: 1rem; }
.form-row .form-group { flex: 1; }
.form-group { margin-bottom: 1rem; }
.form-group label { display: block; margin-bottom: 0.5rem; font-weight: 500; }
.form-group input, .form-group select, .form-group textarea { width: 100%; padding: 0.5rem; border: 1px solid #e5e7eb; border-radius: 0.375rem; }
.modal-footer { display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem; }
.article-content { background: white; border-radius: 0.5rem; padding: 1.5rem; margin-bottom: 1.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
.article-content h3 { color: #4f46e5; margin: 0 0 1rem; }
.article-content p { margin-bottom: 1rem; line-height: 1.6; color: #4b5563; }
.content-wrapper { display: flex; flex-direction: column; gap: 1.5rem; }
</style>
