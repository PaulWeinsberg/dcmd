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
      dark: require('@site/static/img/platforms.svg').default,
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
      dark: require('@site/static/img/templates.svg').default,
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
      dark: require('@site/static/img/lightweight.svg').default,
    },
    description: (
      <>
        Written in Rust, the CLI requires no dependencies and is very small.
        You can use Docker Compose as a drop-in replacement at any time.
      </>
    ),
  },
  {
    title: '100% Customizable',
    svg: {
      light: require('@site/static/img/light/customizable.svg').default,
      dark: require('@site/static/img/customizable.svg').default,
    },
    description: (
      <>
        You can customize the docker-compose file to fit your needs. No dependencies are hidden.
      </>
    ),
  },
];

function Feature({title, svg, description}: FeatureItem) {
  const { colorMode } = useColorMode();
  const Svg = svg[colorMode];
  return (
    <div className={clsx(`col col--3 ${styles.feature}`)}>
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
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
