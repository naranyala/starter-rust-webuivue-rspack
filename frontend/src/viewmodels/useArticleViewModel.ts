import { ref } from 'vue';

export function useArticleViewModel() {
  const article = ref({
    title: 'Understanding Modern Web Development',
    author: 'Tech Writer',
    date: new Date().toLocaleDateString(),
    content: `
      <h2>Introduction</h2>
      <p>Modern web development has evolved significantly over the past decade. From simple static pages to complex single-page applications, the landscape continues to change.</p>
      
      <h2>Key Technologies</h2>
      <p>Today's web developers work with a variety of tools and frameworks. Frontend frameworks like Vue, React, and Angular have become standard for building interactive user interfaces.</p>
      
      <h2>Backend Integration</h2>
      <p>With tools like Rust and WebUI, developers can build high-performance desktop applications using web technologies. This hybrid approach offers the best of both worlds.</p>
      
      <h2>Best Practices</h2>
      <p>Following MVVM architecture helps maintain clean, maintainable code. Separating concerns between models, views, and viewmodels creates more scalable applications.</p>
    `,
  });

  return {
    article,
  };
}
