<template>
  <div class="article-view">
    <div class="article-header">
      <h2>📄 {{ article.title }}</h2>
      <div class="article-meta">
        <span>By {{ article.author }}</span>
        <span>•</span>
        <span>{{ article.date }}</span>
      </div>
    </div>
    <div class="article-body" v-html="article.content"></div>
  </div>
</template>

<script setup>
import { useArticleViewModel } from '../../viewmodels/useArticleViewModel.ts';

const { article } = useArticleViewModel();
</script>

<style scoped>
.article-view { padding: 2rem; max-width: 800px; margin: 0 auto; }
.article-header { margin-bottom: 2rem; border-bottom: 1px solid #e5e7eb; padding-bottom: 1rem; }
.article-header h2 { margin: 0 0 0.5rem; color: #1f2937; }
.article-meta { color: #6b7280; font-size: 0.875rem; display: flex; gap: 0.5rem; }
.article-body { line-height: 1.8; color: #374151; }
.article-body :deep(h2) { color: #4f46e5; margin-top: 2rem; margin-bottom: 1rem; }
.article-body :deep(p) { margin-bottom: 1rem; }
</style>
