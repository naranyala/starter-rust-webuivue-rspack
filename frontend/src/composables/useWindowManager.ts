import { type Ref } from 'vue';

declare global {
  interface Window {
    WinBox: any;
  }
}

export interface WindowInfo {
  id: string;
  title: string;
  minimized: boolean;
  maximized?: boolean;
  winboxInstance: any;
}

export function useWindowManager(activeWindows: Ref<WindowInfo[]>) {
  const openWindow = (title: string, content: string, _icon: string) => {
    if (!window.WinBox) {
      console.error('WinBox is not loaded yet. Please try again in a moment.');
      return;
    }

    const existingWindow = activeWindows.value.find((w) => w.title === title);
    if (existingWindow) {
      if (existingWindow.minimized) {
        existingWindow.winboxInstance.restore();
        existingWindow.minimized = false;
      }
      existingWindow.winboxInstance.focus();
      return;
    }

    const windowId = 'win-' + Date.now();
    let winboxInstance: any;

    winboxInstance = new window.WinBox({
      title: title,
      background: '#1e293b',
      border: 4,
      width: 'calc(100% - 200px)',
      height: '100%',
      x: '200px',
      y: '0',
      minwidth: '300px',
      minheight: '300px',
      max: true,
      min: true,
      mount: document.createElement('div'),
      oncreate: function () {
        this.body.innerHTML = content;
      },
      onfocus: () => {
        const windowInfo = activeWindows.value.find((w) => w.id === windowId);
        if (windowInfo) {
          console.log('Window focused', { windowId, title: windowInfo.title });
        }
      },
      onblur: () => {
        const windowInfo = activeWindows.value.find((w) => w.id === windowId);
        if (windowInfo) {
          console.log('Window blurred', { windowId, title: windowInfo.title });
        }
      },
      onminimize: () => {
        const windowInfo = activeWindows.value.find((w) => w.id === windowId);
        if (windowInfo) {
          windowInfo.minimized = true;
          console.log('Window minimized', { windowId, title: windowInfo.title });
        }
      },
      onrestore: () => {
        const windowInfo = activeWindows.value.find((w) => w.id === windowId);
        if (windowInfo) {
          windowInfo.minimized = false;
          windowInfo.maximized = false;
          console.log('Window restored', { windowId, title: windowInfo.title });
        }
      },
      onmaximize: function () {
        const availableWidth = window.innerWidth - 200;
        const availableHeight = window.innerHeight;
        this.resize(availableWidth, availableHeight);
        this.move(200, 0);

        const windowInfo = activeWindows.value.find((w) => w.id === windowId);
        if (windowInfo) {
          windowInfo.maximized = true;
          console.log('Window maximized', { windowId, title: windowInfo.title });
        }
      },
      onclose: () => {
        const index = activeWindows.value.findIndex((w) => w.id === windowId);
        if (index > -1) {
          activeWindows.value.splice(index, 1);
        }
        console.log('Window closed', { windowId, title });
      },
    });

    activeWindows.value.push({
      id: windowId,
      title,
      minimized: false,
      maximized: false,
      winboxInstance,
    });
  };

  const toggleWindow = (windowInfo: WindowInfo) => {
    if (windowInfo.minimized) {
      windowInfo.winboxInstance.restore();
      windowInfo.minimized = false;
      windowInfo.winboxInstance.focus();
    } else if (windowInfo.maximized) {
      windowInfo.winboxInstance.restore();
      windowInfo.maximized = false;
      windowInfo.winboxInstance.focus();
    } else {
      windowInfo.winboxInstance.minimize();
      windowInfo.minimized = true;
    }
  };

  const closeWindow = (windowInfo: WindowInfo) => {
    windowInfo.winboxInstance.close();
    const index = activeWindows.value.findIndex((w) => w.id === windowInfo.id);
    if (index > -1) {
      activeWindows.value.splice(index, 1);
    }
  };

  const closeAllWindows = () => {
    activeWindows.value.forEach((windowInfo) => {
      windowInfo.winboxInstance.close();
    });
    activeWindows.value = [];
  };

  const hideAllWindows = () => {
    activeWindows.value.forEach((windowInfo) => {
      if (!windowInfo.minimized) {
        windowInfo.winboxInstance.minimize();
        windowInfo.minimized = true;
        windowInfo.maximized = false;
      }
    });
  };

  return {
    openWindow,
    toggleWindow,
    closeWindow,
    closeAllWindows,
    hideAllWindows,
  };
}
