import { Simulation } from '../../simulation.js'
import { IClient, NetworkNClientTestSpec, Suite, Test, TestSpec } from '../../testapi.js'
import { decodeENR } from '../../utils.js'

const testBlocks = [
  {
    contentKey: '0x008faf8b77fedb23eb4d591433ac3643be1764209efa52ac6386e10d1a127e4220',
    content:
      '0x0800000022020000f90217a013ced9eaa49a522d4e7dcf80a739a57dbf08f4ce5efc4edbac86a66d8010f693a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d493479452bc44d5378309ee2abf1539bf71de1b7d7be3b5a0ac4ba3fe45d38b28e2af093024e112851a0f3c72bf1d02b306506e93cd39e26da068d722d467154a4570a7d759cd6b08792c4a1cb994261196b99735222b513bd9a00db8f50b32f1ec33d2546b4aa485defeae3a4e88d5f90fdcccadd6dff516e4b9b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008605af25e8b8e583030d41832fefd88252088455ee029798d783010102844765746887676f312e342e32856c696e7578a0ee8523229bf562950f30ad5a85be3fabc3f19926ee479826d54d4f5f2728c245880a0fb916fd59aad00151c9cb0f67496d0a00000000000000000000000000000000000000000000000026f49629ee7afd5f63095ee154de2dcb05ce9533a86e4cb141466e4329ae78b3b2e02a215d9f52d9f475418efebcbdd7071cddebe56bcf0b76223fb5a16a61e409e296494460fe36174f9834c114fe939876f54d937fc0114a37b31246bb132ad86b2bef5b3f767e8ef08e63e4a19a8a87dfdc7a2e72387a07b0a37ce745618d6d2217e875e41245d8566e0dd0c4aeadc64da5bd029a588326d90b443d9951f5f8b9511802188c5b82016dca842f070d2bd51641d28666f45c58046f2320ab702c91ad236268d46d376bd4096241aacf7bf986ee89efaed55187d5f0ad2081e87927c020d584048b6a1f3066e807b9e62802e70bb1440b9bbc3c095247f5d44211b5fdf73e50ec382b235f6e62ee2d9ff0062bc7739aa6cbdfccd2673ab42bdaa5e765ecbfe9c19b375c59480e421923973eb160894e72881043e42aa91189bedb59d99a56f1208c841f6a21952961a6898e518a9fcc8f15e29f16f89a8635bc53876d4953417ea39ce4330c6d11fae24f0d2df0c504d79d8a6e51c50ee3718948e05af4512da633ea2480622fb7a54a267117dd3f249aa4202650510b6b66cf0020000000000000000000000000000000000000000000000000000000000000',
  },
  {
    contentKey: '0x018faf8b77fedb23eb4d591433ac3643be1764209efa52ac6386e10d1a127e4220',
    content:
      '0x080000007c00000004000000f86e822d85850ba43b740083015f90947c5080988c6d91d090c23d54740f856c69450b29874b04c0f2616400801ba09aaf0e60d53dfb7c34ed51991bd350b8e021185ccc070b4264e209d16df5dc08a03565399bd97800b6d0e9959cd0920702039642b85b37a799391181e0610d6ba9c0',
  },
  {
    contentKey: '0x000c1cf9b3d4aa3e20e12b355416a4e3202da53f54eaaafc882a7644e3e68127ec',
    content:
      '0x0800000022020000f90217a08faf8b77fedb23eb4d591433ac3643be1764209efa52ac6386e10d1a127e4220a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d493479452bc44d5378309ee2abf1539bf71de1b7d7be3b5a0bd0eaff61d52c20e085cb7a7c60b312c792e0b141c5a00e50fd42f8ae1cfe51da09b763cefd23adf252ba87898f7cb8ccc06a4ebddc6be9032648fd55789d4c0b8a0cbb141d48d01bbbf96fb19adff38fb2a6c5e3de40843472a91067ef4f9eac09fb90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008605afdbcd75fd83030d42832fefd88252088455ee029f98d783010102844765746887676f312e342e32856c696e7578a04ddfa646f9a9ec8507af565631322186e2e06347586c9f137383d745ee8bf5958885808f6bbbb2a835014e3f99eb164f6d0a000000000000000000000000000000000000000000000000ad7612b7062b4a509952c3efebb4d692ad5455d26cffafd0ea02ebc6d1478e1c461d97805c3df3728976680dd8c6f99b78029ccf23769849d779b1e332f9e63009e296494460fe36174f9834c114fe939876f54d937fc0114a37b31246bb132ad86b2bef5b3f767e8ef08e63e4a19a8a87dfdc7a2e72387a07b0a37ce745618d6d2217e875e41245d8566e0dd0c4aeadc64da5bd029a588326d90b443d9951f5f8b9511802188c5b82016dca842f070d2bd51641d28666f45c58046f2320ab702c91ad236268d46d376bd4096241aacf7bf986ee89efaed55187d5f0ad2081e87927c020d584048b6a1f3066e807b9e62802e70bb1440b9bbc3c095247f5d44211b5fdf73e50ec382b235f6e62ee2d9ff0062bc7739aa6cbdfccd2673ab42bdaa5e765ecbfe9c19b375c59480e421923973eb160894e72881043e42aa91189bedb59d99a56f1208c841f6a21952961a6898e518a9fcc8f15e29f16f89a8635bc53876d4953417ea39ce4330c6d11fae24f0d2df0c504d79d8a6e51c50ee3718948e05af4512da633ea2480622fb7a54a267117dd3f249aa4202650510b6b66cf0020000000000000000000000000000000000000000000000000000000000000',
  },
  {
    contentKey: '0x010c1cf9b3d4aa3e20e12b355416a4e3202da53f54eaaafc882a7644e3e68127ec',
    content:
      '0x080000007d00000004000000f86f822d86850ba43b740083015f9094c197252baf4a4d2974eab91039594f789a8c207c88017a798d89731c00801ca0825c34f6ddfad0c9fe0e2aa75a3bff9bccc21e81a782fb2a454afb4ad4abac70a0106d3942a42839f74bbbf71b6ff8c5b11082af8b0ff2799cb9b8d14b7fcc9e11c0',
  },
]

const get_block_by_hash = async (test: Test, clients: IClient[]) => {
  const clientsInfo = await Promise.all(
    clients.map(async (client) => {
      const res = await client.rpc.request('discv5_nodeInfo', [])
      return [client.kind, res, decodeENR(res.result.enr)]
    }),
  )
  const store1 = await clients[1].rpc.request('portal_historyStore', [
    testBlocks[0].contentKey,
    testBlocks[0].content,
  ])
  const store2 = await clients[1].rpc.request('portal_historyStore', [
    testBlocks[1].contentKey,
    testBlocks[1].content,
  ])
  const store3 = await clients[1].rpc.request('portal_historyStore', [
    testBlocks[2].contentKey,
    testBlocks[2].content,
  ])
  const store4 = await clients[1].rpc.request('portal_historyStore', [
    testBlocks[3].contentKey,
    testBlocks[3].content,
  ])

  if (!store1.result || !store2.result || !store3.result || !store4.result) {
    test.fatal(`content failed to store`)
  }

  await clients[0].rpc.request('portal_historyPing', [clientsInfo.slice(-1)[0][1].result.enr])
  for await (const [idx, client] of [...clientsInfo.entries()].slice(2)) {
    await clients[1].rpc.request('portal_historyPing', [client[1].result.enr])
    for await (const _client of clientsInfo.slice(idx + 1)) {
      await clients[idx].rpc.request('portal_historyPing', [_client[1].result.enr])
    }
  }
  await new Promise((r) => setTimeout(r, 2000))
  const block1 = await clients[0].rpc.request('eth_getBlockByHash', [
    '0x8faf8b77fedb23eb4d591433ac3643be1764209efa52ac6386e10d1a127e4220',
    true,
  ])
  const block2 = await clients[0].rpc.request('eth_getBlockByHash', [
    '0x0c1cf9b3d4aa3e20e12b355416a4e3202da53f54eaaafc882a7644e3e68127ec',
    true,
  ])

  const expected = {
    block1: {
      number: '0x30d41',
    },
    block2: {
      number: '0x30d42',
    },
  }

  if (!block1.result) {
    test.fatal(`Expected response not received: ${JSON.stringify(block1)}`)
  } else {
    if (block1.result.header.number !== expected.block1.number) {
      test.fatal(`Expected Block ${expected.block1.number}, \n got ${block1.result}`)
    }
  }

  if (!block2.result) {
    test.fatal(`block2: Expected response not received: ${JSON.stringify(block2)}`)
  } else {
    if (block2.result.header.number !== expected.block2.number) {
      test.fatal(`block2: Expected Block ${expected.block2.number}, \n got ${block2.result}`)
    }
  }
}

const run_all_client_tests = async (test: Test) => {
  await test.run(
    new NetworkNClientTestSpec({
      name: 'eth_getBlockByHash',
      description: `eth_getBlockByHash() retrieves blocks 0x30d41 and 0x30d42 from history network with ${
        1 + (await test.sim.client_types()).length
      } nodes`,
      always_run: true,
      run: get_block_by_hash,
    }),
  )
  await test.run(
    new NetworkNClientTestSpec({
      name: 'eth_getBlockByHash',
      description: `eth_getBlockByHash() retrieves blocks 0x30d41 and 0x30d42 from history network with ${
        1 + 2 * (await test.sim.client_types()).length
      } nodes`,
      always_run: true,
      run: get_block_by_hash,
      size: 2,
    }),
  )
  await test.run(
    new NetworkNClientTestSpec({
      name: 'eth_getBlockByHash',
      description: `eth_getBlockByHash() retrieves blocks 0x30d41 and 0x30d42 from history network with ${
        1 + 3 * (await test.sim.client_types()).length
      } nodes`,
      always_run: true,
      run: get_block_by_hash,
      size: 3,
    }),
  )
}

const main = async () => {
  const suite = new Suite(
    'eth_getBlockByHash',
    'This test seeds blocks and headers into a history network, and retrieves them using eth_getBlockByHash',
  )
  suite.add(
    new TestSpec({
      name: 'eth_getBlockByHash',
      description: 'This test suite checks the eth_getBlockByHash RPC method for each client.',
      always_run: true,
      run: run_all_client_tests,
    }),
  )

  const sim = new Simulation(`${process.env['HIVE_SIMULATOR']}`)
  await suite.run(sim)
}

main()
