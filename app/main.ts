import type * as grpcWeb from 'grpc-web';
import { render } from 'solid-js/web';

import { App } from './components/App.js';
import { type ItemResponse, ItemRequest } from './grpc/item_pb.js';
import { ItemClient } from './grpc/ItemServiceClientPb.js';
import './index.css';

render(App, document.body);

const itemService = new ItemClient(
  import.meta.env.MODE ?? 'http://localhost:50051',
);

const request = new ItemRequest();
request.setX(0).setY(0).setColor('').setData('Hello World!');

const call = itemService.unaryItem(
  request,
  { 'custom-header-1': 'value1' },
  (err: grpcWeb.RpcError, response: ItemResponse) => {
    console.log(response.toObject());
  },
);
call.on('status', (status: grpcWeb.Status) => {
  console.log(status);
});
