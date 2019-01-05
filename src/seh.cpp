
struct SEH {
	void* context_pointer;
	void (__stdcall* try_handler)(void*);
	int (__stdcall* except_code)(void*);
	void (__stdcall* except_handler)(void*);
};

extern "C" void __stdcall seh(const SEH* seh) {
	__try {
		seh->try_handler(seh->context_pointer);
	}
	__except(seh->except_code(seh->context_pointer)) {
		seh->except_handler(seh->context_pointer);
	}
}

extern "C" int __stdcall read(const int* i_ptr) {
	return *i_ptr;
}
extern "C" int __stdcall div(int a, int b) {
	return a / b;
}
