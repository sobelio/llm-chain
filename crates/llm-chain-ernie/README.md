# llm_chain_ernie  
  
Baidu Qianfan (also referred to as ernie/Wenxin) platform integration. This enables you to seamlessly access and utilize models hosted on the Baidu Qianfan platform.  
  
Powered by the [[erniebot-rs](https://github.com/chenwanqq/erniebot-rs)] RUST SDK, this integration provides a smooth bridge between your applications and Baidu's AI capabilities. Please note that both this integration and erniebot-rs are community-supported and *not* officially endorsed by Baidu.  

Currently, this integration primarily supports chat models. However, future development plans include adding support for embedding models and conducting extensive testing to ensure compatibility and performance across a wide range of use cases.
  
## Getting Started  
  
1. **Set up Baidu AI Cloud Platform**: Begin by following [[this detailed guide](https://cloud.baidu.com/doc/WENXINWORKSHOP/s/7ltgucw50)] to set up your account and access the necessary services.  
  
2. **Configure Environment Variables**: Before running any applications, ensure you have exported your QIANFAN_AK and QIANFAN_SK as environment variables.  
  
```bash  
export QIANFAN_AK=<your_access_key>  
export QIANFAN_SK=<your_secret_key>
```
3. **Follow the Example**: Refer to the example provided in [simple_generator.rs](./examples/simple_generator.rs). The library includes predefined models such as ErnieBot, ErnieBotTurbo, and Ernie40. However, you have the flexibility to use other models as well.

To utilize a different model, simply identify its name from the API path. For instance, in the URL https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/ernie-4.0-8k-preview, the model name is ernie-4.0-8k-preview. Use this name to specify the desired model in your application code.
