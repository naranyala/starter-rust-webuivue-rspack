// MVVM Architecture Restructure
// =============================

The frontend has been restructured to follow the MVVM (Model-View-ViewModel) architecture pattern.

## New Folder Structure

```
frontend/src/
├── main.ts                    # Entry point - initializes Pinia and Vue
├── App.vue                    # Root View - handles window management
│
├── models/                    # MODEL Layer - Data and business logic
│   ├── types/                 # TypeScript interfaces
│   │   └── index.ts           # User, Product, SystemInfo types
│   ├── stores/                # Pinia stores (state management)
│   │   ├── userStore.ts       # User-related state and actions
│   │   ├── productStore.ts    # Product-related state and actions
│   │   ├── systemStore.ts     # System info state and actions
│   │   └── index.ts
│   └── services/              # Service layer - external communication
│       ├── api/               # API client
│       │   └── apiClient.ts   # useApi composable
│       ├── database/          # Database operations
│       │   └── databaseService.ts
│       ├── eventBus/          # Event bus service
│       │   └── eventBusService.ts
│       └── index.ts
│
├── viewmodels/                # VIEWMODEL Layer - Presentation logic
│   ├── useHomeViewModel.ts    # Home panel logic
│   ├── useUserListViewModel.ts
│   ├── useProductListViewModel.ts
│   ├── useSystemInfoViewModel.ts
│   ├── useEventLogViewModel.ts
│   ├── useArticleViewModel.ts
│   └── index.ts
│
├── views/                     # VIEW Layer - UI components
│   ├── features/
│   │   ├── HomeView.vue       # Home panel view
│   │   ├── UserListView.vue   # User management view
│   │   ├── ProductListView.vue
│   │   ├── SystemInfoView.vue
│   │   ├── EventLogView.vue
│   │   └── ArticleView.vue
│   └── index.ts
│
├── components/                # Shared UI Components
│   ├── common/                # Reusable UI elements
│   │   ├── Button.vue
│   │   ├── Input.vue
│   │   ├── Modal.vue
│   │   └── GlobalErrorModal.vue
│   └── layout/                # Layout components
│       └── Header.vue
│
└── lib/                       # Utilities
    ├── container.ts           # DI Container
    └── logger.ts              # Logger service
```

## Architecture Overview

### Model Layer (`models/`)
Contains all data-related code:
- **Types**: TypeScript interfaces for type safety
- **Stores**: Pinia stores for reactive state management
- **Services**: External communication (API, EventBus, Database)

### ViewModel Layer (`viewmodels/`)
Contains presentation logic:
- Encapsulates state for specific views
- Provides computed properties for derived data
- Contains methods for user actions
- Uses stores and services from the Model layer
- Returns reactive refs for binding to Views

### View Layer (`views/`)
Contains UI components:
- Pure UI rendering (no business logic)
- Binds to ViewModel state
- Emits user actions to ViewModels
- Uses shared components from `components/`

## Migration Guide

### Using the New Structure

1. **Import from new locations**:
   ```typescript
   // Old way (still works for backward compatibility)
   import { useUserStore } from '@/stores/userStore';
   
   // New MVVM way
   import { useUserStore } from '@/models/stores';
   import { useUserListViewModel } from '@/viewmodels';
   ```

2. **Create a View using ViewModel**:
   ```vue
   <template>
     <div class="my-view">
       <p>{{ viewModel.someData }}</p>
       <button @click="viewModel.someAction">Action</button>
     </div>
   </template>
   
   <script setup>
   import { useMyViewModel } from '@/viewmodels';
   const viewModel = useMyViewModel();
   </script>
   ```

3. **Create a new ViewModel**:
   ```typescript
   // viewmodels/useMyViewModel.ts
   import { ref, computed } from 'vue';
   import { useSomeStore } from '@/models/stores';
   
   export function useMyViewModel() {
     const store = useSomeStore();
     const localState = ref('');
     
     const computedValue = computed(() => store.someData + localState.value);
     
     const someAction = () => {
       store.someAction();
     };
     
     return {
       localState,
       computedValue,
       someAction,
       store,
     };
   }
   ```

## Benefits of MVVM

1. **Separation of Concerns**: Clear boundaries between data, logic, and UI
2. **Testability**: ViewModels can be tested independently of Views
3. **Reusability**: Same ViewModel can be used with different Views
4. **Maintainability**: Easier to understand and modify code
5. **Type Safety**: Full TypeScript support across all layers

## Backward Compatibility

The old folder structure (`stores/`, `composables/`) is preserved for backward compatibility. You can gradually migrate components to use the new MVVM structure.

## Key Files

- **Entry Point**: `main.ts` - Updated to use new store imports
- **Root View**: `App.vue` - Updated to use new view imports
- **Types**: `models/types/index.ts` - All TypeScript interfaces
- **ViewModels**: `viewmodels/*.ts` - One per feature
- **Views**: `views/features/*.vue` - UI components using ViewModels
