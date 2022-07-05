#include <iostream>
#include <cstdio>
#include <vector>

#define F(x) for(int i = 0; i < x; i++)
#define pb push_back
#define Feach(x) for(auto i : x)
#define ui unsigned int
#define usi unsigned short int

using namespace std;

typedef struct Nodo {
	bool data;
	Nodo *front;
	Nodo *back;
} Nodo;

class Lista_Circular{
	public:
		Nodo* cabeca;
		Nodo* atual;
	Lista_Circular(){
		this->cabeca = nullptr;
		this->atual = this->cabeca;
	}

	void inserir(bool data){
		if(this->cabeca == nullptr){
			this->cabeca = new Nodo;
			this->cabeca->front = this->cabeca;
			this->cabeca->back = this->cabeca;

			this->atual = this->cabeca;
			this->cabeca->data = data;
		}else{
			Nodo *nodo = new Nodo;
			nodo->data = data;
	
			nodo->front = this->cabeca;
			nodo->back = this->cabeca->back;
	
			this->cabeca->back->front = nodo;
			this->cabeca->back = nodo;
			this->atual = nodo;
		}
	}

	void proximo(){
		this->atual = this->atual->front;
	}

	void voltar(){
		this->atual = this->atual->back;
	}

	Nodo* get_atual(){
		return this->atual;
	}

	void voltar_inicio(){
		this->atual = this->cabeca;
	}
};

int numero_estacas(Nodo *cabeca, Nodo *atual, bool anterior){
	if(cabeca == atual && !atual->data && !anterior){
		atual->data = true;
		return 1;
	}else if(cabeca == atual){
		return 0;
	}

	if(!atual->data && !anterior){
		atual->data = true;
		return 1 + numero_estacas(cabeca, atual->front, true);
	}else{
		return numero_estacas(cabeca, atual->front, atual->data);
	}	
}


int main(){
    ios::sync_with_stdio(0);
    std::cin.tie(0);

	ui n{};
	vector<int> result;

	while(cin >> n){
		if(!n) break;
		// cout << "EAE" << endl;

		Lista_Circular lista;
		bool x{};
		int cont{};

		F(n){
			cin >> x;
			lista.inserir(x);
		}

		lista.voltar_inicio();

		// Objetivo: achar o primeiro 1
		do{
			lista.proximo();
		}while(lista.atual != lista.cabeca && !lista.atual->data);

		Nodo* cabeca = lista.atual, atual;

		cont += numero_estacas(cabeca, cabeca->front, cabeca->data);
		result.pb(cont);
	}

	Feach(result){
		printf("%i\n", i);
	}

    return 0;
}