/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import React, {type CSSProperties, type ReactNode} from 'react';
import clsx from 'clsx';

import styles from './styles.module.scss';

interface Props {
  children: ReactNode;
  minHeight?: number;
  url: string;
  style?: CSSProperties;
  bodyStyle?: CSSProperties;
}

export default function BrowserWindow({
  children,
  minHeight,
  url = 'http://localhost:3000',
  style,
  bodyStyle,
}: Props): JSX.Element {
  return (
    <div className={styles.browserWindow} style={{...style, minHeight}}>
      <div className={styles.browserWindowHeader}>
        <div className={styles.buttons}>
          <span className={styles.dot} style={{background: '#f25f58'}} />
          <span className={styles.dot} style={{background: '#fbbe3c'}} />
          <span className={styles.dot} style={{background: '#58cb42'}} />
        </div>
        <div className={clsx(styles.browserWindowAddressBar, 'text--truncate')}>
          {url}
        </div>
        <div className={styles.browserWindowMenuIcon}>
          <div>
            <span className={styles.bar} />
            <span className={styles.bar} />
            <span className={styles.bar} />
          </div>
        </div>
      </div>

      <div className={styles.browserWindowBody} style={bodyStyle}>
        {children}
      </div>
    </div>
  );
}

// Quick and dirty component, to improve later if needed
export function IframeWindow({url}: {url: string}): JSX.Element {
  return (
    <div style={{padding: 10}}>
      <BrowserWindow
        url={url}
        style={{minWidth: '40vw', maxWidth: 400}}
        bodyStyle={{padding: 0}}>
        <iframe src={url} title={url} style={{width: '100%', height: 300}} />
      </BrowserWindow>
    </div>
  );
}
