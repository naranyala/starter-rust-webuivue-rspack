<template>
  <div class="home-panel">
    <div class="home-header">
      <h1>Welcome to Rust WebUI App</h1>
      <p>Click on a card below to open a window</p>
    </div>
    
    <div class="demo-section">
      <button class="demo-error-btn" @click="triggerDemoError">
        <span class="demo-icon">💥</span>
        Test Error Modal
      </button>
    </div>
    
    <div class="search-container">
      <input 
        v-model="searchQuery" 
        type="text" 
        placeholder="Search cards..." 
        class="search-input"
      />
    </div>
    
    <div class="cards-grid">
      <div 
        v-for="card in filteredCards" 
        :key="card.id"
        class="card"
        @click="openWindow(card)"
      >
        <div class="card-icon">{{ card.icon }}</div>
        <div class="card-content">
          <h3>{{ card.title }}</h3>
          <p>{{ card.description }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useHomeViewModel } from '../../viewmodels/useHomeViewModel.ts';

const { searchQuery, filteredCards, openWindow, triggerDemoError } = useHomeViewModel();
</script>

<style scoped>
.home-panel { padding: 2rem; max-width: 1200px; margin: 0 auto; }
.home-header { text-align: center; margin-bottom: 2rem; }
.home-header h1 { font-size: 2rem; color: #1e293b; margin-bottom: 0.5rem; }
.home-header p { color: #64748b; font-size: 1rem; }
.demo-section { display: flex; justify-content: center; margin-bottom: 2rem; }
.demo-error-btn {
  display: flex; align-items: center; gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  color: white; border: none; border-radius: 8px;
  font-size: 0.95rem; font-weight: 600; cursor: pointer;
  transition: all 0.2s; box-shadow: 0 4px 6px -1px rgba(239, 68, 68, 0.3);
}
.demo-error-btn:hover { transform: translateY(-2px); box-shadow: 0 10px 15px -3px rgba(239, 68, 68, 0.4); }
.demo-error-btn:active { transform: translateY(0); }
.demo-icon { font-size: 1.25rem; }
.search-container { max-width: 400px; margin: 0 auto 2rem; }
.search-input {
  width: 100%; padding: 0.75rem 1rem;
  border: 2px solid #e2e8f0; border-radius: 0.5rem;
  font-size: 1rem; transition: border-color 0.2s;
}
.search-input:focus { outline: none; border-color: #4f46e5; }
.cards-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 1.5rem; }
.card {
  background: white; border-radius: 0.75rem; padding: 1.5rem;
  display: flex; align-items: flex-start; gap: 1rem;
  cursor: pointer; transition: all 0.3s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1); border: 1px solid #e2e8f0;
}
.card:hover { transform: translateY(-4px); box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15); border-color: #4f46e5; }
.card-icon { font-size: 2.5rem; flex-shrink: 0; }
.card-content h3 { font-size: 1.1rem; color: #1e293b; margin-bottom: 0.5rem; }
.card-content p { color: #64748b; font-size: 0.9rem; line-height: 1.5; }
</style>
