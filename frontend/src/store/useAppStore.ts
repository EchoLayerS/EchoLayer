import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { User, Content, PropagationNetwork } from '@echolayer/shared';

interface AppState {
  // User state
  user: User | null;
  isAuthenticated: boolean;
  
  // Content state
  contents: Content[];
  selectedContent: Content | null;
  
  // Network state
  propagationNetwork: PropagationNetwork | null;
  isLoadingNetwork: boolean;
  
  // UI state
  isDarkMode: boolean;
  sidebarOpen: boolean;
  
  // Actions
  setUser: (user: User | null) => void;
  setAuthenticated: (isAuthenticated: boolean) => void;
  setContents: (contents: Content[]) => void;
  setSelectedContent: (content: Content | null) => void;
  setPropagationNetwork: (network: PropagationNetwork | null) => void;
  setLoadingNetwork: (loading: boolean) => void;
  toggleDarkMode: () => void;
  toggleSidebar: () => void;
  addContent: (content: Content) => void;
  updateContent: (id: string, updates: Partial<Content>) => void;
  removeContent: (id: string) => void;
}

export const useAppStore = create<AppState>()(
  devtools(
    (set, get) => ({
      // Initial state
      user: null,
      isAuthenticated: false,
      contents: [],
      selectedContent: null,
      propagationNetwork: null,
      isLoadingNetwork: false,
      isDarkMode: true,
      sidebarOpen: false,

      // Actions
      setUser: (user) => set({ user }),
      
      setAuthenticated: (isAuthenticated) => set({ isAuthenticated }),
      
      setContents: (contents) => set({ contents }),
      
      setSelectedContent: (selectedContent) => set({ selectedContent }),
      
      setPropagationNetwork: (propagationNetwork) => set({ propagationNetwork }),
      
      setLoadingNetwork: (isLoadingNetwork) => set({ isLoadingNetwork }),
      
      toggleDarkMode: () => set((state) => ({ isDarkMode: !state.isDarkMode })),
      
      toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),
      
      addContent: (content) => 
        set((state) => ({ contents: [...state.contents, content] })),
      
      updateContent: (id, updates) =>
        set((state) => ({
          contents: state.contents.map((content) =>
            content.id === id ? { ...content, ...updates } : content
          ),
        })),
      
      removeContent: (id) =>
        set((state) => ({
          contents: state.contents.filter((content) => content.id !== id),
        })),
    }),
    {
      name: 'echolayer-store',
    }
  )
); 