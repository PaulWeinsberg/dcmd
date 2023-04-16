import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.scss';
import { useColorMode } from '@docusaurus/theme-common';


type FeatureItem = {
  title: string;
  svg: { [key in 'dark'|'light']: React.ComponentType<React.ComponentProps<'svg'>>};
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'OS Support',
svg: {
      light: require('@site/static/img/light/platforms.svg').default,
      dark: require('@site/static/img/dark/platforms.svg').default,
    },
    description: (
      <>
        Linux major distribution, MacOS intel or apple chips and Windows 10 & 11 with WSL2 are supported as long as Docker is installed.
      </>
    ),
  },
  {
    title: 'Templates & Examples',
    svg: {
      light: require('@site/static/img/light/templates.svg').default,
      dark: require('@site/static/img/dark/templates.svg').default,
    },
    description: (
      <>
        Start with a template or pick one as example to get up and running quickly.
      </>
    ),
  },
  {
    title: 'Lightweight & Dispensable',
    svg: {
      light: require('@site/static/img/light/lightweight.svg').default,
      dark: require('@site/static/img/dark/lightweight.svg').default,
    },
    description: (
      <>
        Written in Rust, the CLI requires no dependencies and is very small.
        You can use only docker compose as a drop-in replacement at any time.
      </>
    ),
  },
  {
    title: 'Local VHOSTS',
    svg: {
      light: require('@site/static/img/light/domain.svg').default,
      dark: require('@site/static/img/dark/domain.svg').default,
    },
    description: (
      <>
        Use local domains to access your projects. No need to change your hosts file.
      </>
    ),
  },
  {
    title: 'Offline support',
    svg: {
      light: require('@site/static/img/light/offline.svg').default,
      dark: require('@site/static/img/dark/offline.svg').default,
    },
    description: (
      <>
        Once configured, you can work offline. No need to be connected to the internet to resolve local domains.
      </>
    ),
  },
];

function Feature({title, svg, description}: FeatureItem) {
  const { colorMode } = useColorMode();
  const Svg = svg[colorMode];
  return (
    <div className={clsx(`col col--4 ${styles.feature}`)}>
      <div className="text--center margin-bottom--md">
        <Svg className={styles.featureSvg} role="img"  />
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className={clsx(`row ${styles.row}`)}>
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
