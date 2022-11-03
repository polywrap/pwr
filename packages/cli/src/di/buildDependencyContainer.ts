import * as awilix from "awilix";

export const buildDependencyContainer = async(
): Promise<awilix.AwilixContainer<any>> => {
  const container = awilix.createContainer({
    injectionMode: awilix.InjectionMode.PROXY,
  });

  container.register({});

  return container;
};
