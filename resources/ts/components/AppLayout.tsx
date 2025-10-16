import React, { ReactNode } from 'react';
import { app_layout } from './AppLayout.module.css';

export default function AppLayout({ children }: { children: ReactNode }) {
    return (
        <div className={app_layout}>
          { children }
        </div>
    );
}
