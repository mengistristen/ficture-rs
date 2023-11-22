<p align='center'>
  <img src='https://user-images.githubusercontent.com/46630958/106235780-3e29d780-61b0-11eb-9e22-66d4a6d8f7a3.png' height=200 width=200 />
</p>

# ficture-rs

## About

**ficture-rs** is a port of [ficture-terraingen](https://github.com/mengistristen/ficture-terraingen) to Rust. See the original project [here](https://ficture.herokuapp.com).

This project is a command-line tool for generating terrain maps. The bulk of the project is a library that makes generating and modifying these maps easy by providing methods for chaining operations on a grid of cells.

## Usage

```bash
> ficture-generator --filepath ./config.yaml
```

Using the provided config.yaml, this command will generate a map that looks like the following:

![image](https://github.com/mengistristen/ficture-rs/assets/46630958/1fd2f5f2-0263-4e32-8d2c-2219b36022d1)
