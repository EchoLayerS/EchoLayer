/// <reference types="jest" />
/// <reference types="@testing-library/jest-dom" />

declare global {
  namespace jest {
    interface Matchers<R> {
      toBeInTheDocument(): R;
      toHaveClass(className: string): R;
      toHaveAttribute(attr: string, value?: string): R;
    }
  }

  var testUtils: {
    waitForTimeout: (ms: number) => Promise<void>;
  };

  interface Window {
    gtag?: (...args: any[]) => void;
  }
}

export {}; 