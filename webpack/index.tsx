import { h, render, Fragment } from 'preact';
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
    const { data, status } = await axios.get<Package[]>('/api', {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    setDataList(data);
  };

  const packageMap = dataList.map((payload) => {
    return (
      <li key={payload.id} id={`${payload.id}`}>
        <p>{payload.name}</p>
        <p>{payload.age}</p>
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
