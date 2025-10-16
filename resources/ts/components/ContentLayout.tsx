import React, { ReactNode } from 'react';
import {
  content__layout,
  content__layout__content,
} from "./ContentLayout.module.css";

export default function ContentLayout({
  children,
}: {
  children: ReactNode;
}) {

  return (
    <div className={content__layout}>
      <div className={content__layout__content}>{children}</div>
    </div>
  );
}
