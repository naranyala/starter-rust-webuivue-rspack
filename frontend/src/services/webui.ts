import Logger from './logger';

class WebUIService {
  private static instance: WebUIService;
  private logger = Logger;

  static getInstance(): WebUIService {
    if (!WebUIService.instance) {
      WebUIService.instance = new WebUIService();
    }
    return WebUIService.instance;
  }

  async call<T = unknown>(action: string, data?: unknown): Promise<T> {
    this.logger.info(`[WebUI] Calling: ${action}`, data);
    
    try {
      if (window.__webui__) {
        const result = await window.__webui__.call(
          action,
          JSON.stringify(data || {})
        );
        return JSON.parse(result) as T;
      }
      
      this.logger.warn(`[WebUI] Not available, action: ${action}`);
      return { success: false, error: 'WebUI not available' } as T;
    } catch (error) {
      this.logger.error(`[WebUI] Error calling ${action}:`, error);
      throw error;
    }
  }

  bind(event: string, callback: (data: unknown) => void): void {
    if (window.__webui__) {
      window.__webui__.bind(event, (response: string) => {
        try {
          const data = JSON.parse(response);
          callback(data);
        } catch {
          callback(response);
        }
      });
    }
  }
}

export const webui = WebUIService.getInstance();
export default webui;
