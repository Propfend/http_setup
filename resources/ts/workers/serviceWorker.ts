async function addResourcesToCache(resources: RequestInfo[]) {
  const cache = await caches.open("v1");
  await cache.addAll(resources);
};

self.addEventListener("install", (event) => {
  (event as ExtendableEvent).waitUntil(
    addResourcesToCache([
      "/",
      "/favicon.ico",
      // @ts-ignore ${ __PUBLIC_PATH } and ${ __BUILD_ID } represent
      // the relative path to the build and th build ID, respectively.
      `${__PUBLIC_PATH}index_${__BUILD_ID}.js`,
      // @ts-ignore ${ __PUBLIC_PATH } and ${ __BUILD_ID } represent
      // the relative path to the build and th build ID, respectively.
      `${__PUBLIC_PATH}serviceWorker_${__BUILD_ID}.js`
    ]),
  );
});

async function putInCache(request: Request, response: Response) {
  const cache = await caches.open("v1");
  await cache.put(request, response);
};

async function cacheFirst({ request, fallbackUrl, event }: { request: Request, fallbackUrl: string, event: FetchEvent}) {
  try {
    const responseFromNetwork = await fetch(request);
    event.waitUntil(putInCache(request, responseFromNetwork.clone()));
    return responseFromNetwork;
  } catch {
    const fallbackResponse = await caches.match(fallbackUrl);
    if (fallbackResponse) {
      return fallbackResponse;
    }
    return new Response("Network error happened", {
      status: 408,
      headers: { "Content-Type": "text/plain" },
    });
  }
};

self.addEventListener("fetch", (event) => {
  const fetchEvent = event as FetchEvent;

  fetchEvent.respondWith(
    cacheFirst({
      request: fetchEvent.request,
      fallbackUrl: "/gallery/myLittleVader.jpg",
      event: fetchEvent,
    }),
  );
});