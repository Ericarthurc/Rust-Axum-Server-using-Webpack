import { h, render } from 'preact';
import { useState, useEffect } from 'preact/hooks';
import axios from 'axios';

interface Package {
  id: number;
  name: string;
  age: number;
}

const App = (props: any) => {
  const [dataList, setDataList] = useState<Package[]>([]);

  useEffect(() => {
    getData();
  }, []);

  const getData = async () => {
    const { data } = await axios.get<Package[]>('/api', {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    setDataList(data);
  };

  const deleteHandler = (id: number) => {
    setDataList(() => {
      return dataList.filter((payload) => payload.id !== id);
    });
  };

  const packageMap = dataList.map((payload) => {
    return (
      <li key={payload.id}>
        <p>{payload.name}</p>
        <p>{payload.age}</p>
        <button onClick={() => deleteHandler(payload.id)}>Delete</button>
      </li>
    );
  });

  return (
    <div>
      <ul>{packageMap}</ul>
    </div>
  );
};

render(<App />, document.getElementById('root'));
